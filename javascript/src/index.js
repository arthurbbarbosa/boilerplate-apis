import { App } from 'uWebSockets.js'

App()
  .get('/', (response, _request) => {
    response.write("{\"message\": \"Hello, world!\"}")
    response.end()
  })
  .listen(3000, () => {})
