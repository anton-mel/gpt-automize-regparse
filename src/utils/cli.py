# CLI for the LLM-user Pipeline

def get_user_input():
    """ Request Device Register Summary """
    pdf_file_path = input("Enter the path to the PDF file: ").strip()
    page_start = input("Enter the start page (press Enter to start from the beginning): ").strip()
    page_end = input("Enter the end page (press Enter to read until the end): ").strip()

    # Assert input correctness
    if page_start.isdigit(): page_start = int(page_start) - 1
    else: page_start = None
    if page_end.isdigit(): page_end = int(page_end)
    else: page_end = None

    return pdf_file_path, page_start, page_end


