import argparse
import tensorflow as tf
import ctc_utils
import cv2
import numpy as np

import tensorflow.compat.v1 as tf_v1

def find_horizontal_crop_bounds(binary_image, threshold_factor=0.4):
    vertical_projection = np.sum(binary_image, axis=0)
    threshold = np.max(vertical_projection) * threshold_factor
    barline_indices = np.where(vertical_projection > threshold)[0]

    if len(barline_indices) == 0:
        return 0, binary_image.shape[1]

    left = max(barline_indices[0], 0)
    right = min(barline_indices[-1], binary_image.shape[1])
    return left, right

def resize_snippet(image, target_width=800):
    height, width = image.shape[:2]
    if width > target_width:
        scale = target_width / width
        new_width = target_width
        new_height = int(height * scale)
        image = cv2.resize(image, (new_width, new_height), interpolation=cv2.INTER_AREA)
    return image


def split_image_into_systems(image_path, lines_per_system=5):
    image = cv2.imread(image_path, cv2.IMREAD_GRAYSCALE)

    _, binary = cv2.threshold(image, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)

    left, right = find_horizontal_crop_bounds(binary)

    image = image[:, left:right]
    binary = binary[:, left:right]

    projection = np.sum(binary, axis=1)
    threshold = np.max(projection) * 0.5
    line_indices = np.where(projection > threshold)[0]

    max_line_gap = int(image.shape[0] * 0.005)

    line_centers = []
    current_line = []

    for idx in line_indices:
        if not current_line or idx - current_line[-1] < 10:
            current_line.append(idx)
        else:
            # Mittelwert der Gruppe
            line_centers.append(int(np.mean(current_line)))
            current_line = [idx]

    if current_line:
        line_centers.append(int(np.mean(current_line)))

    # Immer 5 Linien als ein System interpretieren
    systems = []
    for i in range(0, len(line_centers), lines_per_system):
        group = line_centers[i:i + lines_per_system]
        if len(group) < lines_per_system:
            break  # Unvollständiges System überspringen

        top = max(group[0] - 40, 0)
        bottom = min(group[-1] + 40, image.shape[0])
        cropped = image[top:bottom, :]
        systems.append(cropped)

    return systems

tf.compat.v1.disable_eager_execution()
tf.config.set_visible_devices([], 'GPU')

parser = argparse.ArgumentParser(description='Decode a music score image with a trained model (CTC).')
parser.add_argument('-image',  dest='image', type=str, required=True, help='Path to the input image.')
parser.add_argument('-model', dest='model', type=str, required=True, help='Path to the trained model.')
parser.add_argument('-vocabulary', dest='voc_file', type=str, required=True, help='Path to the vocabulary file.')
args = parser.parse_args()

tf_v1.reset_default_graph()
sess = tf_v1.InteractiveSession()

# Read the dictionary
dict_file = open(args.voc_file,'r')
dict_list = dict_file.read().splitlines()
int2word = dict()
for word in dict_list:
    word_idx = len(int2word)
    int2word[word_idx] = word
dict_file.close()

# Restore weights
saver = tf_v1.train.import_meta_graph(args.model)
saver.restore(sess,args.model[:-5])

graph = tf_v1.get_default_graph()

input = graph.get_tensor_by_name("model_input:0")
seq_len = graph.get_tensor_by_name("seq_lengths:0")
rnn_keep_prob = graph.get_tensor_by_name("keep_prob:0")
height_tensor = graph.get_tensor_by_name("input_height:0")
width_reduction_tensor = graph.get_tensor_by_name("width_reduction:0")
logits = tf_v1.get_collection("logits")[0]

# Constants that are saved inside the model itself
WIDTH_REDUCTION, HEIGHT = sess.run([width_reduction_tensor, height_tensor])

decoded, _ = tf_v1.nn.ctc_greedy_decoder(logits, seq_len)


line_images = split_image_into_systems(args.image)

with open("./semantic_output", "w", encoding="utf-8") as f:
    for line_img in line_images:
        line_img = ctc_utils.resize(line_img, HEIGHT)
        line_img = ctc_utils.normalize(line_img)
        line_img = np.asarray(line_img).reshape(1, line_img.shape[0], line_img.shape[1], 1)

        seq_lengths = [line_img.shape[2] / WIDTH_REDUCTION]

        prediction = sess.run(decoded,
                              feed_dict={
                                  input: line_img,
                                  seq_len: seq_lengths,
                                  rnn_keep_prob: 1.0,
                              })

        str_predictions = ctc_utils.sparse_tensor_to_strs(prediction)
        for w in str_predictions[0]:
            f.write(int2word[w])
            f.write("\t")
