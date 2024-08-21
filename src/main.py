from utils.pdf_utils import extract_text_from_pdf
from parser.table_parser import extract_table_data
from parser.register_parser import process_registers_summary
from utils.cli import get_user_input

TOKENS_AT_ONCE = 10000

def main():
    # Step #1. Parse PDF2CSV register summary dataset
    pdf_file_path, page_start, page_end = get_user_input()
    # pdf_text = extract_text_from_pdf(pdf_file_path, page_start, page_end)

    # total_length = len(pdf_text)
    # current_position = 0
    
    # while current_position < total_length:
    #     end_position = min(current_position + TOKENS_AT_ONCE, total_length)
    #     chunk_text = pdf_text[current_position:end_position]
    #     extract_table_data(chunk_text)
    #     current_position = end_position

    # Now proccess the CSV table instead
    # Step #2. Parse all detailed information about the registers
    process_registers_summary(pdf_file_path)
    

if __name__ == "__main__":
    main()
