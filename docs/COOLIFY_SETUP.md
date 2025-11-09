# Coolify Setup Guide for joel.val-x.com

Quick setup guide for deploying JOEL documentation to Coolify.

## Quick Start

### Step 1: Create New Application in Coolify

1. Log into your Coolify dashboard
2. Click **"New Resource"** → **"Application"**
3. Select **"Git Repository"**

### Step 2: Connect Repository

- **Repository URL**: `https://github.com/JJ-Dynamite/JOEL.git`
- **Branch**: `develop` (or `main`)
- **Build Pack**: `Node.js`

### Step 3: Configure Build Settings

```
Root Directory: docs
Build Command: npm install && npm run build
Start Command: npm start
Port: 3000
Node Version: 20
```

### Step 4: Add Domain

1. Go to **"Domains"** section
2. Click **"Add Domain"**
3. Enter: `joel.val-x.com`
4. Enable **SSL/TLS** (Let's Encrypt)
5. Set as **Primary Domain**

### Step 5: Environment Variables

Add these environment variables:

```
NODE_ENV=production
NEXT_TELEMETRY_DISABLED=1
PORT=3000
```

### Step 6: Deploy

Click **"Deploy"** and wait for build to complete.

## DNS Configuration

Configure your DNS records:

**Option 1: A Record**
```
Type: A
Name: joel
Value: [Your Coolify Server IP]
TTL: 3600
```

**Option 2: CNAME**
```
Type: CNAME
Name: joel
Value: [Your Coolify Domain]
TTL: 3600
```

## Verification

After deployment:

1. Visit: `https://joel.val-x.com`
2. Check SSL certificate (should be valid)
3. Test navigation and search
4. Verify all pages load correctly

## Auto-Deploy

Coolify will automatically redeploy when you:
- Push to the connected branch
- Manually trigger redeploy in dashboard

## Troubleshooting

**Build fails?**
- Check Node.js version (needs 20+)
- Verify root directory is `docs`
- Check build logs

**Domain not working?**
- Verify DNS propagation (can take up to 48 hours)
- Check SSL certificate status
- Ensure domain is set as primary

**App not starting?**
- Verify port is 3000
- Check environment variables
- Review application logs

## Support

- Coolify Documentation: https://coolify.io/docs
- Next.js Deployment: https://nextjs.org/docs/deployment

---

**Your docs will be live at: https://joel.val-x.com** ✨

