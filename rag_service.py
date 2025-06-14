from langchain_community.embeddings import HuggingFaceEmbeddings
from langchain_community.vectorstores import FAISS
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.chains import RetrievalQA
from langchain_openai import ChatOpenAI
import sqlite3
import json
from datetime import datetime
import sys
import logging
import os
from dotenv import load_dotenv
load_dotenv()

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class PersonalRAG:
    def __init__(self, db_path="personal_info.db"):
        try:
            logger.info("初始化RAG服务...")
            self.conn = sqlite3.connect(db_path)
            self.conn.row_factory = sqlite3.Row
            
            # 从环境变量获取API密钥
            api_key = os.getenv("DEEPSEEK_API_KEY")
            if not api_key:
                logger.error("DEEPSEEK_API_KEY环境变量未设置")
                raise ValueError("请设置DEEPSEEK_API_KEY环境变量")
            
            logger.info("初始化DeepSeek模型...")
            self.llm = ChatOpenAI(
                model_name="deepseek-chat",
                openai_api_key=api_key,
                openai_api_base="https://api.deepseek.com",
                temperature=0.7
            )
            
            logger.info("初始化embeddings...")
            self.embeddings = HuggingFaceEmbeddings(
                model_name="sentence-transformers/all-MiniLM-L6-v2"
            )
            
            logger.info("加载文档...")
            self.load_documents()
            logger.info("RAG服务初始化完成")
        except Exception as e:
            logger.error(f"初始化失败: {str(e)}")
            raise
    
    def format_document(self):
        """从数据库获取并格式化文档"""
        try:
            cursor = self.conn.cursor()
            
            # 获取所有内容
            cursor.execute("""
                SELECT title, content_type, content, tags, created_at, metadata 
                FROM contents 
                ORDER BY created_at DESC
            """)
            
            contents = cursor.fetchall()
            if not contents:
                logger.warning("数据库中没有内容")
                return "没有找到任何内容"
            
            text_parts = []
            
            for content in contents:
                try:
                    # 解析标签
                    tags = json.loads(content['tags'])
                    metadata = json.loads(content['metadata'])
                    
                    # 格式化日期
                    created_at = datetime.fromisoformat(content['created_at'])
                    date_str = created_at.strftime("%Y-%m-%d")
                    
                    # 构建文档片段
                    text_parts.append(f"=== {content['title']} ({date_str}) ===")
                    text_parts.append(f"类型: {content['content_type']}")
                    
                    if tags:
                        text_parts.append(f"标签: {', '.join(tags)}")
                    
                    # 添加内容
                    text_parts.append(content['content'])
                    
                    # 添加元数据中的额外信息（如果有）
                    if metadata:
                        for key, value in metadata.items():
                            if value:
                                text_parts.append(f"{key}: {value}")
                    
                    text_parts.append("\n")  # 添加分隔
                except Exception as e:
                    logger.error(f"处理内容时出错: {str(e)}")
                    continue
            
            return "\n".join(text_parts)
        except Exception as e:
            logger.error(f"格式化文档时出错: {str(e)}")
            raise
    
    def load_documents(self):
        """加载文档并创建向量存储"""
        try:
            text = self.format_document()
            logger.info(f"获取到文档长度: {len(text)} 字符")
            
            # 文本分割
            text_splitter = RecursiveCharacterTextSplitter(
                chunk_size=1000,
                chunk_overlap=200
            )
            texts = text_splitter.split_text(text)
            logger.info(f"文档分割为 {len(texts)} 个片段")
            
            # 创建向量存储
            self.vectorstore = FAISS.from_texts(texts, self.embeddings)
            logger.info("向量存储创建完成")
        except Exception as e:
            logger.error(f"加载文档时出错: {str(e)}")
            raise
    
    def query(self, question: str) -> str:
        """处理查询"""
        try:
            logger.info(f"收到问题: {question}")
            
            qa_chain = RetrievalQA.from_chain_type(
                llm=self.llm,
                chain_type="stuff",
                retriever=self.vectorstore.as_retriever()
            )
            
            result = qa_chain.run(question)
            logger.info(f"生成回答: {result}")
            
            return result
        except Exception as e:
            logger.error(f"查询处理时出错: {str(e)}")
            return f"抱歉，处理您的问题时出现错误: {str(e)}"

    def reload_documents(self):
        logger.info("reload_documents 被调用")
        try:
            self.load_documents()
            logger.info("文档重新加载完成")
            return True
        except Exception as e:
            logger.error(f"重新加载文档时出错: {str(e)}")
            return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python rag_service.py <question> or python rag_service.py --reload")
        sys.exit(1)
    
    try:
        rag = PersonalRAG()
        
        if sys.argv[1] == "--reload":
            logger.info("收到 --reload 参数")
            if rag.reload_documents():
                print("文档重新加载成功")
            else:
                print("文档重新加载失败")
        else:
            # 处理查询
            response = rag.query(sys.argv[1])
            print(response)
    except Exception as e:
        print(f"错误: {str(e)}")
        sys.exit(1)