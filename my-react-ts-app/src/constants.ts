// constants.js
// Great article on various approaches: https://semaphoreci.com/blog/constants-layer-javascript


// API constants
export const APIConstants = Object.freeze({
  GET: 'GET',
  POST: 'POST',
  PUT: 'PUT',
  PATCH: 'PATCH',
  DELETE: 'DELETE',
  APPLICATION_JSON_HEADER: 'application/json',
  // eslint-disable-next-line no-undef
  BACKEND_BASE_URL: process.env.REACT_APP_BACKEND_BASE_URL, // This is loaded from the .env file at React project root
  // ...
})

export const API_URLS = Object.freeze({
  // Should match what's defined in the Platform's API located in the platform api/methods.rs file 
  // TODO: auto generate these from a platform build routine (generating a constants/api_urls.js file specifically for this)

  PLAYER_API: '/api/players',
  PLAYER_SEARCH_API: '/api/players/search',
})
