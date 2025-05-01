#!/usr/bin/env python3
import os
import re
import sys
import time
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin
from selenium import webdriver
from selenium.webdriver.chrome.options import Options

def scroll_to_bottom(driver, pause=2, max_scrolls=50):
    """
    Scrolls the page to the bottom repeatedly, waiting `pause` seconds
    each time, until the scrollHeight stops increasing or we hit max_scrolls.
    """
    last_h = driver.execute_script("return document.body.scrollHeight")
    for _ in range(max_scrolls):
        driver.execute_script("window.scrollTo(0, document.body.scrollHeight);")
        time.sleep(pause)
        new_h = driver.execute_script("return document.body.scrollHeight")
        if new_h == last_h:
            break
        last_h = new_h

def download_sound(mp3_url: str, out_dir: str):
    """
    Downloads an mp3 URL into out_dir, skipping if the file already exists.
    """
    fname = os.path.basename(mp3_url)
    dest = os.path.join(out_dir, fname)
    if os.path.exists(dest):
        print(f"⏭ Skipping existing {fname}")
        return
    print(f"⬇️  Downloading {fname}")
    r = requests.get(mp3_url, stream=True)
    r.raise_for_status()
    with open(dest, "wb") as f:
        for chunk in r.iter_content(8192):
            f.write(chunk)
    print(f"✅ Saved {fname}")

def main():
    if len(sys.argv) != 3:
        print("Usage: python download_instants.py <page_url> <output_dir>")
        sys.exit(1)

    page_url, out_dir = sys.argv[1], sys.argv[2]
    os.makedirs(out_dir, exist_ok=True)

    # 1) Launch headless Chrome
    opts = Options()
    opts.headless = True
    driver = webdriver.Chrome(options=opts)
    driver.get(page_url)

    # 2) Scroll until no new instants load
    scroll_to_bottom(driver)

    # 3) Grab the full HTML
    html = driver.page_source
    driver.quit()

    # 4) Parse and find every <div class="instant">
    soup = BeautifulSoup(html, "html.parser")
    container = soup.find(id="instants_container")
    instants = (
        container.find_all("div", class_="instant")
        if container
        else soup.find_all("div", class_="instant")
    )

    if not instants:
        print("⚠️  No instants found.")
        return

    # 5) Extract each MP3 URL and download
    for div in instants:
        mp3_path = None
        # some divs carry it as data-sound
        if div.has_attr("data-sound"):
            mp3_path = div["data-sound"]
        else:
            # else look for the first onclick="play('…mp3',…)"
            btn = div.find("button", onclick=True)
            if btn:
                m = re.search(r"['\"](/media/sounds/[^'\"]+\.mp3)['\"]",
                              btn["onclick"])
                if m:
                    mp3_path = m.group(1)

        if not mp3_path:
            print("⚠️  Could not find mp3 URL in an instant, skipping.")
            continue

        full_url = urljoin(page_url, mp3_path)
        download_sound(full_url, out_dir)

if __name__ == "__main__":
    main()
