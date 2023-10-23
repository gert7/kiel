from http.server import BaseHTTPRequestHandler, HTTPServer
import string
import subprocess

hostname = "0.0.0.0"
server_port = 8196

def execute_hour():
    output = subprocess.run(["/usr/local/bin/kiel", "hour-force", "--enact"],
                            capture_output=True)
    if output.returncode == 0:
        stdout = output.stdout
        print(f"{stdout}")
    else:
        stderr = output.stderr
        print(f"{stderr}")

class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path.startswith("/service/"):
            echo = self.path.split("/")[2]
            print("Kiel service request received!")
            self.send_response(200)
            self.end_headers()
            self.wfile.write(bytes(f"Kiel says hello, {echo}!", "utf-8"))
        elif self.path.startswith("/hour"):
            print("Hour executed")
            execute_hour()

if __name__ == "__main__":
    web_server = HTTPServer((hostname, server_port), MyServer)
    print("Running server...")

    try:
        web_server.serve_forever()
    except KeyboardInterrupt:
        pass

    web_server.server_close()
