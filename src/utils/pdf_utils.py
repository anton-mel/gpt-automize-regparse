import PyPDF2
import os

def extract_text_from_pdf(pdf_path, page_start=None, page_end=None):
    pdf_path = pdf_path.strip('\'"')
    pdf_path = os.path.normpath(pdf_path)

    if not os.path.isfile(pdf_path):
        raise FileNotFoundError(f"The file at {pdf_path} does not exist.")
        
    with open(pdf_path, 'rb') as file:
        reader = PyPDF2.PdfReader(file)
        text = ''

        total_pages = len(reader.pages)
        if page_start is None:
            page_start = 0
        if page_end is None or page_end > total_pages:
            page_end = total_pages

        for page_num in range(page_start, page_end):
            text += reader.pages[page_num].extract_text()
    
    return text
