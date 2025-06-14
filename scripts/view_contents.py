import sqlite3
import json
import sys


def view_contents(db_path):
    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    cursor = conn.cursor()
    
    cursor.execute("SELECT * FROM contents ORDER BY created_at DESC")
    rows = cursor.fetchall()
    
    for row in rows:
        print(f"\n标题: {row['title']}")
        print(f"类型: {row['content_type']}")
        print(f"内容: {row['content']}")
        print(f"标签: {row['tags']}")
        print(f"创建时间: {row['created_at']}")
        print(f"元数据: {row['metadata']}")
        print("-" * 50)
    
    conn.close()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python view_contents.py <db_path>")
        sys.exit(1)
    
    view_contents(sys.argv[1])