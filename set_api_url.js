var replace = require('replace-in-file');

const options = {
  files: 'src/environments/environment.ts',
  from: /{API_URL}/g,
  to: process.env.API_URL,
  allowEmptyPaths: false
};

try {
  let changedFiles = replace.sync(options);
  console.log(options);
  console.log(changedFiles);
  console.log('API_URL set: ' + process.env.API_URL);
}

catch (error) {
  console.error('Error occurred:', error);
}

