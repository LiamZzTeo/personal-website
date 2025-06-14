import sqlite3
import json
import sys
from datetime import datetime

def init_db(db_path):
    """初始化数据库"""
    conn = sqlite3.connect(db_path)
    with open('scripts/init_db.sql', 'r') as f:
        conn.executescript(f.read())
    return conn

def import_data(db_path, json_file):
    """导入JSON数据到数据库"""
    conn = init_db(db_path)
    cursor = conn.cursor()
    
    with open(json_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    for content in data['contents']:
        cursor.execute("""
            INSERT INTO contents (title, content_type, content, tags, metadata)
            VALUES (?, ?, ?, ?, ?)
        """, (
            content['title'],
            content['content_type'],
            content['content'],
            json.dumps(content['tags']),
            json.dumps(content['metadata'])
        ))
    
    conn.commit()
    conn.close()

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python import_data.py <db_path> <json_file>")
        sys.exit(1)
    
    db_path = sys.argv[1]
    json_file = sys.argv[2]
    import_data(db_path, json_file)