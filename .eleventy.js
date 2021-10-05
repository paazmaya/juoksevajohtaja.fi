module.exports = (eleventyConfig) => {
  eleventyConfig.addPassthroughCopy("src/icons");

  return {
    dir: {
      includes: "partials",
      layouts: "templates",
      input: "src",
      output: "dist"
    }
  };
};
