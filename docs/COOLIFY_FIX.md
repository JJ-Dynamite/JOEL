# Coolify Deployment Fix

## Issue

Coolify was trying to clone the `main` branch which didn't exist. The repository uses `develop` and `master` branches.

## Solution

A `main` branch has been created and pushed to match `develop`. Now Coolify can deploy from `main`.

## Updated Deployment Steps

1. **In Coolify Dashboard:**
   - Repository: `https://github.com/JJ-Dynamite/JOEL.git`
   - **Branch**: `main` ✅ (now available)
   - Build Pack: `Node.js`

2. **Build Settings:**
   ```
   Root Directory: docs
   Build Command: npm install && npm run build
   Start Command: npm start
   Port: 3000
   ```

3. **Domain**: `joel.val-x.com`

4. **Deploy**

## Branch Structure

- `main` - Production branch (synced with develop)
- `develop` - Development branch
- `master` - Git flow production branch

## Auto-Sync

To keep `main` in sync with `develop`:

```bash
git checkout main
git merge develop
git push origin main
```

Or set up a GitHub Action to auto-sync branches.

## Alternative: Use Develop Branch

If you prefer to deploy from `develop`:

1. In Coolify, change branch to `develop`
2. All other settings remain the same

---

**The `main` branch is now available and ready for deployment!** 🚀

