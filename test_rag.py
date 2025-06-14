# test_rag.py
from rag_service import PersonalRAG

def test_rag():
    try:
        rag = PersonalRAG()
        response = rag.query("请介绍一下你自己")
        print("回答:", response)
    except Exception as e:
        print("错误:", str(e))

if __name__ == "__main__":
    test_rag()