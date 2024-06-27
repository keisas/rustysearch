import React, { useState } from 'react';
import './App.css';

function App() {
  const [query, setQuery] = useState('');
  const [searchType, setSearchType] = useState('sequential');
  const [useML, setUseML] = useState(false);
  const [results, setResults] = useState([]);
  const [elapsedTime, setElapsedTime] = useState(null);

  const handleSearch = async () => {
    try {
      const response = await fetch(`http://localhost:8080/search?query=${query}&_search_type=${searchType}&_use_ml=${useML}`);
      if (!response.ok) {
        console.error('Network response was not ok');
        throw new Error('Network response was not ok');
      }

      const data = await response.json();
      setResults(data.results);
      setElapsedTime(data.elapsed_time);
    } catch (error) {
      console.error('There was a problem with the fetch operation:', error);
    }
  };

  return (
    <div className='continer'>
      <h1>AI Powered Search</h1>
      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder='Enter your search query'
      />
      <div>
        <label>
          Search Type:
          <select value={searchType} onChange={(e) => setSearchType(e.target.value)}>
            <option value="sequential">Sequential</option>
            <option value="index">Index</option>
            <option value="fulltext">Fulltext</option>
          </select>
        </label>
      </div>
      <div className="checkbox-container">
        <label>
          Use ML:
          <input
            type="checkbox"
            checked={useML}
            onChange={(e) => setUseML(e.target.checked)}
          />
        </label>
      </div>
      <button onClick={handleSearch}>Search</button>
      {elapsedTime !== null && (
        <p>Search took {elapsedTime.toFixed(10)} seconds</p>
      )}
      <ul>
        {results.map((result) => (
          <li key={result.id}>
            <h2>{result.title}</h2>
            <p>{result.description}</p>
            <p>Relecance score: {result.relevance_score}</p>
          </li>
        ))}
      </ul>
    </div>
  )
}

export default App;
