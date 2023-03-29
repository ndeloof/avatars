import os
import mysql.connector

class DBManager:
    def __init__(self, database='avatar', host="db", user="root", password_file=None):
        pf = open(password_file, 'r')
        self.connection = mysql.connector.connect(
            user=user, 
            password=pf.read(),
            host=host,
            database=database,
            auth_plugin='mysql_native_password'
        )
        pf.close()
        self.cursor = self.connection.cursor()
    
    def populate_db(self):
        self.cursor.execute('DROP TABLE IF EXISTS clothing')
        self.cursor.execute('CREATE TABLE clothing (company VARCHAR(255) PRIMARY KEY, color VARCHAR(255))')
        self.cursor.execute('INSERT INTO clothing (company, color) VALUES ("tilt", "#20BA31")')
        self.cursor.execute('INSERT INTO clothing (company, color) VALUES ("docker", "#086DD7")')
        
        self.connection.commit()
    
    def query_clothing(self, company):
        self.cursor.execute('SELECT color FROM clothing WHERE company = %s', (company,))
        rec = '#000'
        for c in self.cursor:
            rec = c[0]
        return 