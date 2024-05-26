import socket
import random
import time
import psutil
import time

def get_pc_time():
    return time.strftime("%m-%d %H:%M:%S", time.localtime())

def get_cpu_temperature():
    temperature = psutil.sensors_temperatures()
    if "coretemp" in temperature:
        return temperature["coretemp"][0].current
    else:
        return None

def get_ram_usage():
    return psutil.virtual_memory().percent

def send_random_numbers(ip, port):
    # Create a socket object
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Rest of the code
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    # ...

    try:
        # Connect to the server
        client_socket.connect((ip, port))
        print(f"Connected to {ip}:{port}")

        while True:
            # Generate a random number
            cpu_temperature = get_cpu_temperature()
            ram_usage = get_ram_usage()
            system_time = get_pc_time()

            # Convert the number to bytes
            data = f"Time: {system_time}\nCPU: {cpu_temperature}\nRAM {ram_usage}%".encode()

            # Send the data to the server
            client_socket.sendall(data)
            print(f"Sent: {data.decode()}")

            # Wait for 3 seconds
            time.sleep(2)
    except:
        print("An error occurred")
        client_socket.close()
# Usage example
ip_address = "192.168.1.88"  # Replace with the desired IP address
port_number = 1234  # Replace with the desired port number

send_random_numbers(ip_address, port_number)