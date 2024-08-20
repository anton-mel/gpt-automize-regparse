from utils.pdf_utils import extract_text_from_pdf
from parser.table_parser import extract_table_data
from utils.cli import get_user_input

def main():
    # basic example of the PDF-gpt data parse
    pdf_file_path, page_start, page_end = get_user_input()
    pdf_text = extract_text_from_pdf(pdf_file_path, page_start, page_end)
    table_data = extract_table_data(pdf_text)
    
    print("Extracted Table Data:")
    print(table_data)

if __name__ == "__main__":
    main()
