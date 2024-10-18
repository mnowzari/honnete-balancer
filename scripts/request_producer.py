import requests as rq
import time

def send_test_requests(r):
    try:
        req = rq.get(r)
    except Exception as e:
        print(f"Exception has occurred {e}")

def main():
    sleeptime = 0.001
    req_per_sec = 1000 / (sleeptime * 1000)
    print (f"Sleep time: {sleeptime}")
    print (f"# of requests/second: {req_per_sec}")
    while True:
        time.sleep(sleeptime)
        request_string = 'HTTP://127.0.0.1:9700/liine_restaurant/v1/get_open_restaurants/07-03-2024%2011AM/?format=json'
        send_test_requests(request_string)

        time.sleep(sleeptime)
        request_string = 'HTTP://127.0.0.1:9700/liine_restaurant/v1/get_open_restaurants/05-25-2023%2004AM/?format=api'
        send_test_requests(request_string)

        time.sleep(sleeptime)
        request_string = 'HTTP://127.0.0.1:9700/liine_restaurant/v1/get_open_restaurants/10-09-2024%2012PM/?format=json'
        send_test_requests(request_string)
main()
