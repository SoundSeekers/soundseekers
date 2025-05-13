import argparse
import tensorflow as tf
import ctc_utils
import cv2
import numpy as np

import tensorflow.compat.v1 as tf_v1

def split_image_into_systems(image_path, lines_per_system=5):
    import cv2
    import numpy as np

    image = cv2.imread(image_path, cv2.IMREAD_GRAYSCALE)

    # Binarize the image
    _, binary = cv2.threshold(image, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)

    # Compute horizontal projection
    projection = np.sum(binary, axis=1)
    threshold = np.max(projection) * 0.4
    line_indices = np.where(projection > threshold)[0]

    # Gruppiere nahe beieinanderliegende Pixelzeilen als einzelne Linie
    line_positions = []
    current_line = []
    for idx in line_indices:
        if not current_line or idx - current_line[-1] < 10:
            current_line.append(idx)
        else:
            line_positions.append(int(np.mean(current_line)))
            current_line = [idx]
    if current_line:
        line_positions.append(int(np.mean(current_line)))

    # Jetzt immer 5 Linien als ein System zusammenfassen
    systems = []
    for i in range(0, len(line_positions), lines_per_system):
        group = line_positions[i:i+lines_per_system]
        if len(group) < lines_per_system:
            break

        top = max(group[0] - 20, 0)
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
for i, line_img in enumerate(line_images):
    cv2.imshow(f'System {i+1}', line_img)
    cv2.waitKey(0)
    cv2.destroyAllWindows()

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
