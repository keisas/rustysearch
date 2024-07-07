import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [elapsedTime, setElapsedTime] = useState(null);
  const [currentPage, setCurrentPage] = useState(1);
  const resultsPerPage = 50;

  const handleSearch = async () => {
    try {
      const response = await fetch(`http://localhost:8080/search?query=${query}`);
      if (!response.ok) {
        console.error('Network response was not ok');
        throw new Error('Network response was not ok');
      }

      const data = await response.json();
      setResults(data.results);
      setElapsedTime(data.elapsed_time);
      setCurrentPage(1);  // Reset to first page on new search
    } catch (error) {
      console.error('There was a problem with the fetch operation:', error);
    }
  };

  const handlePageChange = (pageNumber) => {
    setCurrentPage(pageNumber);
    window.scrollTo(0, 0);  // Scroll to top on page change
  };

  // Calculate displayed results based on pagination
  const indexOfLastResult = currentPage * resultsPerPage;
  const indexOfFirstResult = indexOfLastResult - resultsPerPage;
  const currentResults = results.slice(indexOfFirstResult, indexOfLastResult);

  // Calculate total pages
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
      <button onClick={handleSearch}>Search</button>
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