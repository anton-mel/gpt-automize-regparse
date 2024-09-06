from parser.table_parser import extract_table_data
from utils.pdf_utils import extract_text_from_pdf
from generate_code import generate_struct_code
from utils.cli import get_user_input

TOKENS_AT_ONCE = 1000

def main():
    # basic example of the PDF-gpt data parse
    # pdf_file_path, page_start, page_end = get_user_input()
    # pdf_text = extract_text_from_pdf(pdf_file_path, page_start, page_end)
    # extract_table_data(pdf_text)

    # total_length = len(pdf_text)
    # current_position = 0
    
    # while current_position < total_length:
    #     end_position = min(current_position + TOKENS_AT_ONCE, total_length)
    #     chunk_text = pdf_text[current_position:end_position]
    #     extract_table_data(chunk_text)
    #     current_position = end_position

    # Now given the CSV file I will manually write a struct code
    generate_struct_code("output.rs")

if __name__ == "__main__":
    main()
