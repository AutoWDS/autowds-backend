/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export', // Disabled: incompatible with middleware
    trailingSlash: true,
    images: {
        unoptimized: true
    }
}

module.exports = nextConfig
