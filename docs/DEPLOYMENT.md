# Deployment Guide

This guide explains how to deploy the JOEL documentation to Coolify at `joel.val-x.com`.

## Prerequisites

- Coolify instance running and accessible
- Domain `joel.val-x.com` configured in your DNS
- Git repository access

## Deployment Methods

### Method 1: Deploy from Git Repository (Recommended)

1. **Connect Repository in Coolify:**
   - Go to your Coolify dashboard
   - Click "New Resource" → "Application"
   - Select "Git Repository"
   - Connect to: `https://github.com/JJ-Dynamite/JOEL.git`
   - Set branch to: `main` (or `develop`)

2. **Configure Build Settings:**
   - **Build Pack**: Node.js
   - **Build Command**: `cd docs && npm install && npm run build`
   - **Start Command**: `cd docs && npm start`
   - **Port**: `3000`
   - **Root Directory**: `docs`

3. **Configure Domain:**
   - Add domain: `joel.val-x.com`
   - Enable SSL (Let's Encrypt)
   - Set as primary domain

4. **Environment Variables:**
   ```
   NODE_ENV=production
   NEXT_TELEMETRY_DISABLED=1
   PORT=3000
   ```

5. **Deploy:**
   - Click "Deploy"
   - Coolify will build and deploy automatically

### Method 2: Deploy with Docker

1. **Build Docker Image:**
   ```bash
   cd docs
   docker build -t joel-docs .
   ```

2. **Run Container:**
   ```bash
   docker run -d \
     --name joel-docs \
     -p 3000:3000 \
     -e NODE_ENV=production \
     joel-docs
   ```

3. **Configure in Coolify:**
   - Create new Docker Compose resource
   - Use the provided `coolify.yml`
   - Configure domain and SSL

### Method 3: Manual Deployment

1. **Build Locally:**
   ```bash
   cd docs
   npm install
   npm run build
   ```

2. **Upload to Server:**
   - Upload the `docs/` directory to your server
   - Ensure Node.js 20+ is installed

3. **Run Production Server:**
   ```bash
   cd docs
   npm start
   ```

4. **Configure Reverse Proxy (Nginx):**
   ```nginx
   server {
       listen 80;
       server_name joel.val-x.com;

       location / {
           proxy_pass http://localhost:3000;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }
   }
   ```

## Coolify-Specific Configuration

### Build Settings

```
Build Pack: Node.js
Node Version: 20
Build Command: cd docs && npm install && npm run build
Start Command: cd docs && npm start
Port: 3000
Root Directory: docs
```

### Environment Variables

Add these in Coolify's environment variables section:

```
NODE_ENV=production
NEXT_TELEMETRY_DISABLED=1
PORT=3000
```

### Domain Configuration

1. In Coolify, go to your application settings
2. Navigate to "Domains"
3. Add domain: `joel.val-x.com`
4. Enable SSL/TLS (Let's Encrypt)
5. Set as primary domain

### DNS Configuration

Ensure your DNS is configured:

```
Type: A
Name: joel
Value: [Your Coolify server IP]
TTL: 3600
```

Or use CNAME:

```
Type: CNAME
Name: joel
Value: [Your Coolify domain]
TTL: 3600
```

## Post-Deployment

### Verify Deployment

1. Visit `https://joel.val-x.com`
2. Check that all pages load correctly
3. Verify search functionality
4. Test dark mode toggle
5. Check mobile responsiveness

### Monitoring

- Check Coolify logs for any errors
- Monitor application health
- Set up alerts if needed

## Troubleshooting

### Build Fails

- Check Node.js version (requires 20+)
- Verify all dependencies are installed
- Check build logs in Coolify

### Domain Not Working

- Verify DNS configuration
- Check SSL certificate status
- Ensure domain is set as primary in Coolify

### Application Not Starting

- Check port configuration (should be 3000)
- Verify environment variables
- Check application logs in Coolify

## Updating Documentation

### Automatic Updates

If connected to Git repository:
1. Push changes to `develop` or `main` branch
2. Coolify will automatically rebuild and redeploy

### Manual Updates

1. Make changes to documentation
2. Commit and push to repository
3. Trigger redeploy in Coolify dashboard

## SSL/HTTPS

Coolify automatically handles SSL certificates via Let's Encrypt:
- Enable SSL in domain settings
- Certificates auto-renew
- HTTPS redirect is automatic

## Performance Optimization

The Next.js app is optimized for production:
- Static generation where possible
- Image optimization enabled
- Code splitting automatic
- Caching headers configured

## Support

For issues:
- Check Coolify logs
- Review Next.js build output
- Verify domain and DNS settings
- Contact Coolify support if needed

---

**Your documentation will be live at: https://joel.val-x.com** 🚀

