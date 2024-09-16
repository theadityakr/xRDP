import socket
import threading
import cv2
import numpy as np
import tkinter as tk
from PIL import Image, ImageTk

class RDPClient:
    def __init__(self, host='38.107.67.232', port=3000):
        self.host = host
        self.port = port
        self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.root = tk.Tk()
        self.root.title("RDP Client")
        self.canvas = tk.Canvas(self.root)
        self.canvas.pack(fill=tk.BOTH, expand=True)

    def start(self):
        self.client_socket.connect((self.host, self.port))
        receive_thread = threading.Thread(target=self.receive_screen)
        receive_thread.start()
        self.root.bind("<Motion>", self.send_mouse_event)
        self.root.bind("<Button-1>", self.send_click_event)
        self.root.bind("<Key>", self.send_keyboard_event)
        self.root.mainloop()

    def receive_screen(self):
        try:
            while True:
                size = int.from_bytes(self.client_socket.recv(4), byteorder='big')
                image_data = b''
                while len(image_data) < size:
                    packet = self.client_socket.recv(size - len(image_data))
                    if not packet:
                        return
                    image_data += packet
                image = cv2.imdecode(np.frombuffer(image_data, dtype=np.uint8), cv2.IMREAD_COLOR)
                self.update_canvas(image)
        except Exception as e:
            print(f"Error receiving screen: {e}")

    def update_canvas(self, image):
        height, width = image.shape[:2]
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        photo = ImageTk.PhotoImage(image=Image.fromarray(image))
        self.canvas.config(width=width, height=height)
        self.canvas.create_image(0, 0, anchor=tk.NW, image=photo)
        self.canvas.image = photo

    def send_mouse_event(self, event):
        self.client_socket.sendall(b'm')
        self.client_socket.sendall(f"{event.x},{event.y}".encode())

    def send_click_event(self, event):
        self.client_socket.sendall(b'c')

    def send_keyboard_event(self, event):
        self.client_socket.sendall(b'k')
        self.client_socket.sendall(event.char.encode())

if __name__ == "__main__":
    client = RDPClient()
    client.start()