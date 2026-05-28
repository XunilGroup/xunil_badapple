import cv2, os
from PIL import Image

video_length = 218

ASCII_CHARS = '0123456789'


def handle_frame():
    image = Image.open("output.jpg").convert("L")
    pixels_in_image = list(image.getdata())
    pixels_to_chars = [ASCII_CHARS[pixel_value * (len(ASCII_CHARS) - 1) // 255] for pixel_value in pixels_in_image]

    return "".join(pixels_to_chars)

def main():
    video = cv2.VideoCapture("fixed_video.mp4")
    time_count = 0

    frames = []

    while time_count <= video_length * 1000:
        video.set(0, time_count)

        success, image = video.read()

        if success:
            cv2.imwrite("output.jpg", image)

        frame_ascii = handle_frame()

        os.remove("output.jpg")

        frames.append(frame_ascii)

        time_count += 100

    with open("bad_apple_ascii.txt", "w") as file:
        file.write("NEWFRAME".join(frames))

if __name__ == "__main__":
    main()
