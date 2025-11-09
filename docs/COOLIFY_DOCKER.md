# Coolify Docker Deployment

## Quick Setup with Dockerfile

The repository now includes a Dockerfile in the root directory that Coolify will automatically detect.

### Configuration in Coolify

1. **Repository**: `https://github.com/JJ-Dynamite/JOEL.git`
2. **Branch**: `main`
3. **Dockerfile**: Automatically detected (root directory)
4. **Port**: `3000`
5. **Domain**: `joel.val-x.com`

### What the Dockerfile Does

- Uses Node.js 20 Alpine (lightweight)
- Multi-stage build for optimization
- Installs dependencies from `docs/package.json`
- Builds Next.js application
- Creates standalone production image
- Runs on port 3000

### No Additional Configuration Needed

Coolify will:
- ✅ Automatically detect the Dockerfile
- ✅ Build the Docker image
- ✅ Run the container
- ✅ Handle port mapping
- ✅ Set up SSL/HTTPS

### Environment Variables (Optional)

You can add these in Coolify if needed:

```
NODE_ENV=production
NEXT_TELEMETRY_DISABLED=1
PORT=3000
```

### Verify Deployment

After deployment:
1. Visit `https://joel.val-x.com`
2. Check that documentation loads
3. Verify SSL certificate
4. Test navigation and search

---

**The Dockerfile is ready - just deploy in Coolify!** 🐳

