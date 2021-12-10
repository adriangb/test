import requests


def are_headers_equal(file_url: str, *file_urls: str) -> bool:
    raise NotImplementedError

def are_files_equal(file_url: str, *file_urls: str) -> bool:
    raise NotImplementedError


if __name__ == "__main__":
    files = [f.strip() for f in open("files.txt").readlines()]
    print(are_headers_equal(*files))
    print(are_files_equal(*files))
