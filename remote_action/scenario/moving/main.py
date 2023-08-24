import asyncio
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer

from forester_http.client import *

hostName = "localhost"
serverPort = 10001


class MyServer(BaseHTTPRequestHandler):

    def do_POST(self):
        if self.path == "/move_to":
            body = self.rfile.read(int(self.headers["Content-Length"]))
            req = RemoteActionRequest.from_bytes(body)

            self.send_response(200)
            self.send_header("Content-Type", "application/json;charset=UTF-8")
            self.end_headers()

            if req.tick == 5:
                client = ForesterHttpClient(req.serv_url)
                client.put("calculated", False)
                client.new_trace_event(req.tick, "Bump!. Recalculate")

            if req.tick > 10:
                self.wfile.write(json.dumps("Success").encode("utf-8"))
            else:
                self.wfile.write(json.dumps("Running").encode("utf-8"))

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
