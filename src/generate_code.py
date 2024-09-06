import csv
import os
import re
from parser.register_parser import snake_case
from parser.handle_offset import query_openai_offset_expression

# File paths
registers_summary_path = './registers_summary.csv'
registers_info_path = './register_info'

def calculate_padding(offset1, offset2):
    """Calculate the padding size between two offsets."""
    return offset2 - offset1 - 4

def parse_register_info(file_path):
    """Parse the detailed register info CSV file to determine field size."""
    fields = []
    with open(file_path, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            fields.append(row)
    return fields

def calculate_register_size(fields):
    """Calculate register size based on fields."""
    size = 0
    for field in fields:
        bit_range = field['Bit(s)']
        bits = [int(x) for x in re.findall(r'\d+', bit_range)]
        size = max(size, bits[-1] + 1)
        
    # Similarly to HAL code, round up mod 4 bytes
    return (size + 31) // 32 * 4 

def extract_base_offset(offset_str):
    """Extract the base part of the offset before the first '+' or non-hex character."""
    match = re.match(r'^0x[0-9A-Fa-f]+', offset_str)
    if match:
        base_offset = match.group(0)  # Extracts the hex part before any '+'
        return int(base_offset, 16)
    else:
        raise ValueError(f"Invalid offset format: {offset_str}")

def handle_offset(offset_str, rw_type):
    """Handle offset expressions or simple addresses."""
    if re.match(r'^0x[0-9A-Fa-f]+$', offset_str):
        # If it's a simple address, parse it directly
        return int(offset_str, 16), None
    else:
        # If it's a complex expression, use OpenAI to process it
        return None, query_openai_offset_expression(offset_str, rw_type)

def generate_struct_code(output_rust_file):
    """Generate Rust struct code based on the register summary and detailed info."""
    struct_code = "#[derive(FromBytes)]\n#[repr(C)]\npub struct Registers {\n"
    
    previous_offset = None

    # Read and sort the register summary by the base part of the Offset field
    with open(registers_summary_path, 'r') as f:
        reader = csv.DictReader(f)
        rows = sorted(reader, key=lambda row: extract_base_offset(row['Offset']))

    for idx, row in enumerate(rows):
        offset_str = row['Offset']
        name = snake_case(row['Name'])
        rw_type = row['RW']

        offset, field_type_declaration = handle_offset(offset_str, rw_type)

        if offset is None:
            if field_type_declaration:
                struct_code += f"    {name}: {field_type_declaration},\n"
            else:
                print(f"Skipping entry with non-parsable offset: {offset_str}")
            continue

        # Determine field type based on RW type
        if field_type_declaration is None:
            if rw_type == "RW":
                field_type = "Volatile<u32>"
            elif rw_type == "RO":
                field_type = "ReadOnly<u32>"
            elif rw_type == "WO":
                field_type = "WriteOnly<u32>"
            else:
                field_type = "Reserved<u32>"

            register_info_file = os.path.join(registers_info_path, f"{name}.csv")
            if os.path.exists(register_info_file):
                fields = parse_register_info(register_info_file)
                register_size = calculate_register_size(fields)
                field_type_declaration = f"[{field_type}; {register_size // 4}]"
            else:
                field_type_declaration = field_type

        if previous_offset is not None:
            # Calculate the padding required
            padding = calculate_padding(previous_offset, offset)
            if padding > 0:
                struct_code += f"    _padding{idx-1}: [u8; {padding}], // 0x{previous_offset + 4:X} - 0x{offset - 1:X}\n\n"

        struct_code += f"    /// {row['Name']}\n"
        struct_code += f"    {name}: {field_type_declaration}, // 0x{offset:X}\n"
        previous_offset = offset

    struct_code += "}\n"

    # Write the struct code to a Rust file
    with open(output_rust_file, 'w') as rust_file:
        rust_file.write(struct_code)

    print(f"Done! Rust struct has been written to {output_rust_file}")
