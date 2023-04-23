# Exif-Viewer-in-Rust
A small program to view EXIF data of images.
It prints EXIF data to console and writes it to the file also.
It uses crate 'exif' to extract EXIF data. 

--------------

What is EXIF?

EXIF (Exchangeable Image File Format).
These files store important data about photographs. Almost all digital cameras create these data files each time you snap a new picture. An EXIF file holds all the information about the image itself — such as the exposure level, where you took the photo and any settings you used.
This makes it easier to filter photos on your storage device by particular image characteristics. Also it’s useful for photographers to make catalog your images easier.

Abobe information is taken from following URL

https://www.adobe.com/in/creativecloud/file-types/image/raster/exif-file.html#:~:text=EXIF%20(Exchangeable%20Image%20File%20Format,and%20any%20settings%20you%20used.

--------------

How to run the progam?

For System requirements please refer Application_Requirements.txt

Just clone the repo, build and run.

cargo build

cargo run

Two sample images and their output exif data is present inside project folder. 

-----------

Output 


1. Output Exif Data Screen 1

![ExifOutput1](https://user-images.githubusercontent.com/86361080/233816188-82b64192-d191-4928-bdb5-ee443677170f.png)

2. Output Exif Data Screen 2

![ExifOutput2](https://user-images.githubusercontent.com/86361080/233816203-7e5e7bac-3f2d-46af-98e3-69e7c145e8cc.png)

-----------
