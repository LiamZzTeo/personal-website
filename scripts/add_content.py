import sqlite3
import json
import sys
from datetime import datetime

def add_content(db_path, title, content_type, content, tags, metadata=None):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    if metadata is None:
        metadata = {}
    
    cursor.execute("""
        INSERT INTO contents (title, content_type, content, tags, metadata)
        VALUES (?, ?, ?, ?, ?)
    """, (
        title,
        content_type,
        content,
        json.dumps(tags),
        json.dumps(metadata)
    ))
    
    conn.commit()
    conn.close()

if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python add_content.py <db_path> <title> <content_type> <content> [tags...]")
        sys.exit(1)
    
    db_path = sys.argv[1]
    title = sys.argv[2]
    content_type = sys.argv[3]
    content = sys.argv[4]
    tags = sys.argv[5:] if len(sys.argv) > 5 else []
    
    add_content(db_path, title, content_type, content, tags)