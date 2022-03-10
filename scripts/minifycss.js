const fs = require('fs');
const child_process = require('child_process');

const buildDirectory = './dist';
fs.readdir(buildDirectory, { encoding: 'utf-8' }, (err, files) => {
  if (err) {
    throw new Error(
      'There has been an error reading directory: ' +
        buildDirectory +
        ' - ' +
        err
    );
  }

  const css = files.filter((file) => file.includes('.css'));
  const getUncachedCss = css.filter((file) => !file.includes('-'));
  const fileName = getUncachedCss.join('').replace('.css', '');

  css.forEach((file) => {
    child_process.exec(
      `npx postcss ./dist/${file} > ./dist/${fileName}.css`,
      (err) => {
        if (err) {
          console.error(err);
        }
      }
    );
  });
});
