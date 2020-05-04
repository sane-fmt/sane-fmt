import process from 'process'
import { main } from './index'
main(process).catch(error => {
  console.error('Unexpected Exception')
  console.error(error)
  return process.exit(1)
})
