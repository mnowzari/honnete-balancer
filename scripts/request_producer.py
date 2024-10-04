import requests as rq
import time

def send_test_requests():
    req = rq.get('HTTP://127.0.0.1:9700/request')
    print(req)
    print(req.content)
    req = rq.post('HTTP://127.0.0.1:9700/request')
    print(req)
    print(req.content)
    print()

def main():
    print("Sending as many requests as possible...")
    while True:
        # print('Sleeping')
        # time.sleep(1)
        send_test_requests()
main()
