import * as application from '../lib/pkg';

const result = application.add(3, 8);

document.querySelectorAll("body")[0].innerText = new String(result);
console.log()
