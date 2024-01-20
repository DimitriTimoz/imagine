import easyocr
import sys
from PIL import Image
import csv

def main():
    # get the path argument
    path = sys.argv[1]
    # check if the path is a file
    try:
        with Image.open(path) as img:
            reader = easyocr.Reader(['fr','en'], gpu=True) 
            result = reader.readtext(path, detail=1)
            # print to csv
            for row in result:
                box = row[0]
                p1 = box[0]
                p2 = box[1]
                p3 = box[2]
                p4 = box[3]
                print(box, file=sys.stderr)
                text = row[1]
                confidence = row[2]
                box = f"({p1}-{p2}),({p3}-{p4})"
                print(box, text, confidence, sep=';')
            
            return 0    
    except IOError:
        return 1
    

if __name__ == '__main__':
    sys.exit(main())  