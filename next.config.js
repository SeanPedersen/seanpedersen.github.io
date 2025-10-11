/** @type {import('next').NextConfig} */
const withBundleAnalyzer = require('@next/bundle-analyzer')({
  enabled: process.env.ANALYZE === 'true',
});

const nextConfig = {
  basePath: "",
  output: "export",
  reactStrictMode: true,

  // Production optimizations
  compiler: {
    removeConsole: process.env.NODE_ENV === 'production' ? {
      exclude: ['error', 'warn'],
    } : false,
  },

  // Use SWC minifier for better performance
  swcMinify: true,

  // Optimize images (required for static export)
  images: {
    unoptimized: true,
  },
};

module.exports = withBundleAnalyzer(nextConfig);
