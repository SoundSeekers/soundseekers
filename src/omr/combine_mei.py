
import argparse
import uuid
from lxml import etree
from copy import deepcopy

MEI_NS = "http://www.music-encoding.org/ns/mei"
NSMAP = {"mei": MEI_NS}
etree.register_namespace("mei", MEI_NS)

XML_ID_KEY = '{http://www.w3.org/XML/1998/namespace}id'

def generate_unique_id(existing_ids, base_id):
    new_id = base_id
    while new_id in existing_ids:
        new_id = f"{base_id}-{uuid.uuid4().hex[:8]}"
    existing_ids.add(new_id)
    return new_id

def collect_existing_ids(element, existing_ids):
    for key, val in element.attrib.items():
        if 'id' in key:
            existing_ids.add(val)
    for child in element:
        collect_existing_ids(child, existing_ids)

def update_ids(element, existing_ids):
    for key in list(element.attrib.keys()):
        if 'id' in key:
            old_id = element.attrib[key]
            new_id = generate_unique_id(existing_ids, old_id)
            element.attrib[key] = new_id
    for child in element:
        update_ids(child, existing_ids)

def combine_mei_files(input_files, output_file, mode='sequential'):
    base_tree = etree.parse(input_files[0])
    base_root = base_tree.getroot()

    existing_ids = set()
    collect_existing_ids(base_root, existing_ids)
    update_ids(base_root, existing_ids)

    roots = [etree.parse(f).getroot() for f in input_files[1:]]

    if mode == 'sequential':
        base_section = base_root.find(".//mei:section", namespaces=NSMAP)
        if base_section is None:
            raise ValueError("No <section> found in base MEI file.")

        for idx, root in enumerate(roots, start=1):
            section_in = root.find(".//mei:section", namespaces=NSMAP)
            if section_in is None:
                raise ValueError(f"No <section> in input file {input_files[idx]}")

            for measure in section_in.findall("./mei:measure", namespaces=NSMAP):
                new_measure = deepcopy(measure)
                update_ids(new_measure, existing_ids)
                base_section.append(new_measure)

    elif mode == 'parallel':
        all_roots = [base_root] + roots
        measure_lists = []
        max_measures = 0

        for idx, root in enumerate(all_roots):
            section_in = root.find(".//mei:section", namespaces=NSMAP)
            if section_in is None:
                raise ValueError(f"No <section> found in file {input_files[idx]}")
            measures = section_in.findall("./mei:measure", namespaces=NSMAP)
            measure_lists.append(measures)
            max_measures = max(max_measures, len(measures))

        base_section = base_root.find(".//mei:section", namespaces=NSMAP)
        for old_measure in base_section.findall("./mei:measure", namespaces=NSMAP):
            base_section.remove(old_measure)

        for measure_index in range(max_measures):
            new_measure = etree.Element(f"{{{MEI_NS}}}measure", n=str(measure_index + 1))
            staff_1 = etree.Element(f"{{{MEI_NS}}}staff", n="1")
            staff_2 = etree.Element(f"{{{MEI_NS}}}staff", n="2")

            for file_idx, measures in enumerate(measure_lists):
                if measure_index < len(measures):
                    original_measure = measures[measure_index]
                    staff_elements = original_measure.findall("./mei:staff", namespaces=NSMAP)

                    if file_idx == 0:
                        target_staff = staff_1
                    elif file_idx == 1:
                        target_staff = staff_2
                    else:
                        continue

                    for staff in staff_elements:
                        layers = staff.findall("./mei:layer", namespaces=NSMAP)
                        for layer in layers:
                            layer_copy = deepcopy(layer)
                            update_ids(layer_copy, existing_ids)
                            target_staff.append(layer_copy)

            if len(staff_1):
                new_measure.append(staff_1)
            if len(staff_2):
                new_measure.append(staff_2)

            update_ids(new_measure, existing_ids)
            base_section.append(new_measure)
    else:
        raise ValueError("Invalid mode. Use 'sequential' or 'parallel'.")

    base_tree.write(output_file, pretty_print=True, xml_declaration=True, encoding='UTF-8')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Combine MEI files.")
    parser.add_argument("input_files", nargs="+", help="List of input MEI files.")
    parser.add_argument("--mode", choices=["sequential", "parallel"], default="sequential", help="Combination mode.")
    parser.add_argument("--output", required=True, help="Output MEI file name.")
    args = parser.parse_args()

    combine_mei_files(args.input_files, args.output, mode=args.mode)
