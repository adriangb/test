import requests


def are_headers_equal(file_url: str, *file_urls: str) -> bool:
    raise NotImplementedError

def are_files_equal(file_url: str, *file_urls: str) -> bool:
    raise NotImplementedError


if __name__ == "__main__":
    assert are_headers_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/2.csv") == True
    assert are_headers_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/3.csv") == False
    assert are_headers_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/4.csv") == True

    assert are_files_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/2.csv") == True
    assert are_files_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/3.csv") == False
    assert are_files_equal("https://www.adriangb.com/test/1.csv", "https://www.adriangb.com/test/4.csv") == False
