import socket
import threading
import mss
import numpy as np
import cv2
import pyautogui

class RDPServer:
    def __init__(self, host='0.0.0.0', port=5000):
        self.host = host
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.bind((self.host, self.port))
        self.server_socket.listen(1)
        self.sct = mss.mss()

    def start(self):
        print(f"Server listening on {self.host}:{self.port}")
        while True:
            client_socket, addr = self.server_socket.accept()
            print(f"Connection from {addr}")
            client_thread = threading.Thread(target=self.handle_client, args=(client_socket,))
            client_thread.start()

    def handle_client(self, client_socket):
        try:
            while True:
                # Capture screen
                screen = np.array(self.sct.grab(self.sct.monitors[0]))
                _, encoded_image = cv2.imencode('.jpg', screen, [cv2.IMWRITE_JPEG_QUALITY, 50])
                client_socket.sendall(len(encoded_image).to_bytes(4, byteorder='big'))
                client_socket.sendall(encoded_image)

                # Receive input events
                event_type = client_socket.recv(1).decode()
                if event_type == 'm':  # mouse event
                    x, y = map(int, client_socket.recv(8).decode().split(','))
                    pyautogui.moveTo(x, y)
                elif event_type == 'c':  # click event
                    pyautogui.click()
                elif event_type == 'k':  # keyboard event
                    key = client_socket.recv(1).decode()
                    pyautogui.press(key)
        except Exception as e:
            print(f"Error handling client: {e}")
        finally:
            client_socket.close()

if __name__ == "__main__":
    server = RDPServer()
    server.start()