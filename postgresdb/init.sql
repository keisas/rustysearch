CREATE TABLE IF NOT EXISTS articles (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tsv tsvector
);

INSERT INTO articles (title, description) VALUES
('This is Docker', 'Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.'),
('Python Programming', 'Python is an interpreted, high-level and general-purpose programming language. Python''s design philosophy emphasizes code readability with its notable use of significant indentation.'),
('Machine Learning', 'Machine learning is a field of artificial intelligence that uses statistical techniques to give computer systems the ability to learn from data, without being explicitly programmed.'),
('Deep Learning', 'Deep learning is a class of machine learning algorithms that uses multiple layers of neural networks to model complex patterns in data.'),
('Artificial Intelligence', 'Artificial intelligence (AI) is intelligence demonstrated by machines, in contrast to the natural intelligence displayed by humans and animals.'),
('Data Science', 'Data science is an inter-disciplinary field that uses scientific methods, processes, algorithms and systems to extract knowledge and insights from structured and unstructured data.'),
('Big Data', 'Big data refers to data sets that are too large or complex to be dealt with by traditional data-processing software.'),
('Cloud Computing', 'Cloud computing is the on-demand availability of computer system resources, especially data storage and computing power, without direct active management by the user.'),
('Blockchain', 'Blockchain is a growing list of records, called blocks, that are linked together using cryptography.'),
('Internet of Things', 'The Internet of things (IoT) describes the network of physical objects that are embedded with sensors, software, and other technologies for the purpose of connecting and exchanging data with other devices and systems over the Internet.');

UPDATE articles SET tsv = to_tsvector('english', coalesce(title, '') || ' ' || coalesce(description, ''));

CREATE INDEX IF NOT EXISTS idx_articles_tsv ON articles USING gin(tsv);