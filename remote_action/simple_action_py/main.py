import json
from http.server import BaseHTTPRequestHandler, HTTPServer

from forester_http.client import *

hostName = "localhost"
serverPort = 10001


class MyServer(BaseHTTPRequestHandler):

    def do_GET(self):
        """Respond to a GET request."""
        if self.path == "/":
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            self.wfile.write("OK".encode("utf-8"))

        else:
            self.send_error(404)

    def do_POST(self):

        if self.path == "/action":
            content_length = int(self.headers["Content-Length"])
            # get body as json and deserialize it to RemoteActionRequest
            body = json.loads(self.rfile.read(content_length))
            req = RemoteActionRequest(body["tick"], [RtArgument(arg["name"], arg["value"]) for arg in body["args"]],
                                      body["serv_url"])

            client = ForesterHttpClient(req.serv_url)
            client.put("test", "test")

            self.send_response(200)
            self.send_header("Content-Type", "application/json;charset=UTF-8")
            self.end_headers()

            self.wfile.write(json.dumps("Success").encode("utf-8"))

        else:
            self.send_error(404)


if __name__ == "__main__":
    webServer = HTTPServer((hostName, serverPort), MyServer)
    print("Server started http://%s:%s" % (hostName, serverPort))

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")
