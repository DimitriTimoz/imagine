import easyocr
import sys
from PIL import Image

def main():
    # get the path argument
    path = sys.argv[1]

    # check if the path is a file
    try:
        with Image.open(path) as img:
            reader = easyocr.Reader(['fr','en'], gpu=True) 

            result = reader.readtext(path, detail=1)
            print(result)
            return 0    
    except IOError:
        return 1
    

if __name__ == '__main__':
    sys.exit(main())  