import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [elapsedTime, setElapsedTime] = useState(null);
  const [currentPage, setCurrentPage] = useState(1);
  const resultsPerPage = 50;
  const [mode, setMode] = useState("keyword");


  const handleSearch = async () => {
    try {
      const response = await fetch(`https://rustysearch.yonecoding.com/search?query=${query}&mode=${mode}`);
      if (!response.ok) {
        const errorText = await response.text();
        console.error('Network response was not ok');
        console.error('Status:', response.status);
        console.error('Status Text:', response.statusText);
        console.error('Response Body:', errorText);
        throw new Error('Network response was not ok');
      }
      const data = await response.json();
      setResults(data.results);
      setElapsedTime(data.elapsed_time);
      setCurrentPage(1);
    } catch (error) {
      console.error('There was a problem with the fetch operation:', error);
    }
  };

  const handlePageChange = (pageNumber) => {
    setCurrentPage(pageNumber);
    window.scrollTo(0, 0);
  };

  const indexOfLastResult = currentPage * resultsPerPage;
  const indexOfFirstResult = indexOfLastResult - resultsPerPage;
  const currentResults = results.slice(indexOfFirstResult, indexOfLastResult);

  const totalPages = Math.ceil(results.length / resultsPerPage);

  return (
    <div className='container'>
      <h1>AI Powered Search</h1>
      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder='Enter your search query'
      />
      <div
  style={{
    display: 'flex',
    alignItems: 'center',
    gap: '10px',
    marginTop: '20px',
  }}
>

  <button
    onClick={() => setMode("keyword")}
    style={{
      padding: '8px 12px',
      backgroundColor: mode === "keyword" ? "#804000" : "#eee",
      color: mode === "keyword" ? "white" : "#333",
      border: mode === "keyword" ? "none" : "1px solid #ccc",
      borderRadius: "6px",
    }}
  >
    全文
  </button>
  <button
    onClick={() => setMode("semantic")}
    style={{
      padding: '8px 12px',
      backgroundColor: mode === "semantic" ? "#804000" : "#eee",
      color: mode === "semantic" ? "white" : "#333",
      border: mode === "semantic" ? "none" : "1px solid #ccc",
      borderRadius: "6px",
    }}
  >
    ベクトル
  </button>

  <div style={{ flexGrow: 1 }} />

  <button
    onClick={handleSearch}
    style={{
      padding: '10px 20px',
      backgroundColor: "#5c3317",
      color: "white",
      border: "none",
      borderRadius: "8px",
    }}
  >
    検索実行
  </button>
</div>
      {elapsedTime !== null && (
        <p>Search took {elapsedTime.toFixed(10)} seconds</p>
      )}
      {totalPages > 1 && (
        <div className="pagination">
          {Array.from({ length: totalPages }, (_, index) => (
            <button
              key={index + 1}
              onClick={() => handlePageChange(index + 1)}
              className={currentPage === index + 1 ? 'active' : ''}
            >
              {index + 1}
            </button>
          ))}
        </div>
      )}
      <ul>
        {currentResults.map((result) => (
          <li key={result.isbn}>
            <div className="book-info">
              <h2>{result.title}</h2>
              <p>{result.author}</p>
              <p>Year: {result.publication_year}</p>
              <p>Publisher: {result.publisher}</p>
              <p>Relevance score: {result.relevance_score}</p>
            </div>
            <img src={result.image_url} alt={result.title} />
          </li>
        ))}
      </ul>
      {totalPages > 1 && (
        <div className="pagination">
          {Array.from({ length: totalPages }, (_, index) => (
            <button
              key={index + 1}
              onClick={() => handlePageChange(index + 1)}
              className={currentPage === index + 1 ? 'active' : ''}
            >
              {index + 1}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;