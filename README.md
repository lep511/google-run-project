# Google Cloud Run Deployment Guide

## Prerequisites

1. **Google Cloud CLI** installed and authenticated
2. **Docker** installed locally
3. **OpenAI API Key** from OpenAI platform

## Step 1: Project Setup

Create or update your `Cargo.toml`:

```toml
[package]
name = "google-run"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
```

## Step 2: Configure Google Cloud

```bash
# Set your project ID
export PROJECT_ID="your-project-id"
gcloud config set project $PROJECT_ID

# Enable required APIs
gcloud services enable cloudbuild.googleapis.com
gcloud services enable run.googleapis.com
gcloud services enable artifactregistry.googleapis.com
```

## Step 3: Build and Deploy

### Option A: Direct Deploy (Recommended)

```bash
# Deploy directly from source code
gcloud run deploy ai-chat-app \
  --source . \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars="OPENAI_API_KEY=your-openai-api-key-here" \
  --memory=512Mi \
  --cpu=1 \
  --concurrency=100 \
  --timeout=300 \
  --max-instances=10
```

### Option B: Using Docker Registry

```bash
# Create Artifact Registry repository
gcloud artifacts repositories create ai-chat-repo \
  --repository-format=docker \
  --location=us-central1

# Configure Docker authentication
gcloud auth configure-docker us-central1-docker.pkg.dev

# Build and tag image
docker build -t us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:latest .

# Push to registry
docker push us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:latest

# Deploy from registry
gcloud run deploy ai-chat-app \
  --image us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars="OPENAI_API_KEY=your-openai-api-key-here" \
  --memory=512Mi \
  --cpu=1
```

## Step 4: Secure Environment Variables (Recommended)

Instead of passing the API key directly, use Google Secret Manager:

```bash
# Create secret
echo -n "your-openai-api-key-here" | gcloud secrets create openai-api-key --data-file=-

# Deploy with secret
gcloud run deploy ai-chat-app \
  --source . \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-secrets="OPENAI_API_KEY=openai-api-key:latest" \
  --memory=512Mi \
  --cpu=1
```

## Step 5: Custom Domain (Optional)

```bash
# Map custom domain
gcloud run domain-mappings create \
  --service ai-chat-app \
  --domain your-domain.com \
  --region us-central1
```

## Step 6: Monitoring and Scaling

Create a `cloudbuild.yaml` for CI/CD:

```yaml
steps:
  # Build the container image
  - name: 'gcr.io/cloud-builders/docker'
    args: ['build', '-t', 'us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:$COMMIT_SHA', '.']
  
  # Push the container image to Artifact Registry
  - name: 'gcr.io/cloud-builders/docker'
    args: ['push', 'us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:$COMMIT_SHA']
  
  # Deploy container image to Cloud Run
  - name: 'gcr.io/google.com/cloudsdktool/cloud-sdk'
    entrypoint: gcloud
    args:
      - 'run'
      - 'deploy'
      - 'ai-chat-app'
      - '--image'
      - 'us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:$COMMIT_SHA'
      - '--region'
      - 'us-central1'
      - '--platform'
      - 'managed'
      - '--allow-unauthenticated'

images:
  - 'us-central1-docker.pkg.dev/$PROJECT_ID/ai-chat-repo/ai-chat-app:$COMMIT_SHA'
```

## Environment Variables Configuration

Your application will automatically receive these environment variables:

- `PORT` - Set by Cloud Run (usually 8080)
- `OPENAI_API_KEY` - Your OpenAI API key
- `GOOGLE_CLOUD_PROJECT` - Your project ID
- `K_SERVICE` - Service name
- `K_REVISION` - Revision name

## Cost Optimization

```bash
# Set minimum instances to 0 for cost savings
gcloud run services update ai-chat-app \
  --region us-central1 \
  --min-instances=0 \
  --max-instances=10
```

## Monitoring

```bash
# View logs
gcloud run services logs tail ai-chat-app --region=us-central1

# View service details
gcloud run services describe ai-chat-app --region=us-central1
```

## Security Best Practices

1. **Use Secret Manager** for sensitive data
2. **Enable IAM authentication** for production
3. **Set up VPC connector** if accessing private resources
4. **Configure CORS** properly for web clients
5. **Use HTTPS** (enabled by default on Cloud Run)

## Troubleshooting

Common issues and solutions:

1. **Build failures**: Check Rust version and dependencies
2. **Port binding**: Ensure your app binds to `0.0.0.0:$PORT`
3. **Memory limits**: Increase if getting OOM errors
4. **Cold starts**: Consider minimum instances > 0
5. **API limits**: Implement rate limiting and error handling

## Testing

```bash
# Get service URL
SERVICE_URL=$(gcloud run services describe ai-chat-app --region=us-central1 --format='value(status.url)')

# Test the endpoint
curl $SERVICE_URL

# Test chat API
curl -X POST $SERVICE_URL/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, AI!"}'
```

Your AI chat application will be available at the provided Cloud Run URL!