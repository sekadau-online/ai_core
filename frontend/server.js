const express = require('express');
const axios = require('axios');
const dotenv = require('dotenv');
const path = require('path');
const bodyParser = require('body-parser');
const cookieParser = require('cookie-parser');

// Load environment variables
dotenv.config();

const app = express();
const PORT = process.env.PORT || 8080;
const BACKEND_URL = process.env.BACKEND_URL || 'http://127.0.0.1:3000';
const BEARER_TOKEN = process.env.BEARER_TOKEN;

// Middleware
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));
app.use(cookieParser());
app.use(express.static(path.join(__dirname, 'public')));
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));

// Axios instance with auth
const apiClient = axios.create({
    baseURL: BACKEND_URL,
    headers: {
        'Authorization': `Bearer ${BEARER_TOKEN}`,
        'Content-Type': 'application/json'
    }
});

// Helper function to handle API errors
const handleApiError = (error) => {
    if (error.response) {
        return {
            success: false,
            message: error.response.data?.message || 'API Error',
            status: error.response.status
        };
    }
    return {
        success: false,
        message: error.message || 'Connection Error',
        status: 500
    };
};

// ============================================
// ROUTES
// ============================================

// Home page
app.get('/', async (req, res) => {
    try {
        const healthResponse = await axios.get(`${BACKEND_URL}/health`);
        res.render('index', {
            apiStatus: 'online',
            backendUrl: BACKEND_URL
        });
    } catch (error) {
        res.render('index', {
            apiStatus: 'offline',
            backendUrl: BACKEND_URL
        });
    }
});

// Experiences page
app.get('/experiences', async (req, res) => {
    try {
        const response = await apiClient.get('/experiences');
        res.render('experiences', {
            experiences: response.data.data || [],
            message: response.data.message
        });
    } catch (error) {
        const errorData = handleApiError(error);
        res.render('experiences', {
            experiences: [],
            error: errorData.message
        });
    }
});

// Create experience page
app.get('/create', (req, res) => {
    res.render('create');
});

// Create experience POST
app.post('/api/experiences', async (req, res) => {
    try {
        const { content, source, metadata } = req.body;
        const response = await apiClient.post('/experiences', {
            content,
            source: source || 'user',
            metadata: metadata || null
        });
        res.json(response.data);
    } catch (error) {
        res.status(error.response?.status || 500).json(handleApiError(error));
    }
});

// Search experiences
app.get('/search', async (req, res) => {
    try {
        const query = req.query.q || '';
        if (!query) {
            return res.render('search', { results: [], query: '' });
        }
        
        const response = await apiClient.get(`/experiences/search?q=${encodeURIComponent(query)}`);
        res.render('search', {
            results: response.data.data || [],
            query: query
        });
    } catch (error) {
        res.render('search', {
            results: [],
            query: req.query.q || '',
            error: handleApiError(error).message
        });
    }
});

// Statistics page
app.get('/stats', async (req, res) => {
    try {
        const response = await apiClient.get('/stats');
        res.render('stats', {
            stats: response.data.data
        });
    } catch (error) {
        res.render('stats', {
            stats: null,
            error: handleApiError(error).message
        });
    }
});

// Personality page
app.get('/personality', (req, res) => {
    res.render('personality');
});

// Update personality POST
app.post('/api/personality', async (req, res) => {
    try {
        const { input, response: responseText } = req.body;
        const response = await apiClient.post('/personality', {
            input,
            response: responseText
        });
        res.json(response.data);
    } catch (error) {
        res.status(error.response?.status || 500).json(handleApiError(error));
    }
});

// Decision page
app.get('/decision', async (req, res) => {
    try {
        const query = req.query.q;
        let response;
        
        if (query) {
            response = await apiClient.get(`/decision/query?q=${encodeURIComponent(query)}`);
        } else {
            response = await apiClient.get('/decision');
        }
        
        res.render('decision', {
            decision: response.data.data,
            query: query || ''
        });
    } catch (error) {
        res.render('decision', {
            decision: null,
            query: req.query.q || '',
            error: handleApiError(error).message
        });
    }
});

// Interact page
app.get('/interact', async (req, res) => {
    try {
        const response = await apiClient.get('/interact');
        res.render('interact', {
            interaction: response.data.data
        });
    } catch (error) {
        res.render('interact', {
            interaction: null,
            error: handleApiError(error).message
        });
    }
});

// Reflect page
app.get('/reflect', async (req, res) => {
    try {
        const response = await apiClient.get('/reflect');
        res.render('reflect', {
            reflection: response.data.data
        });
    } catch (error) {
        res.render('reflect', {
            reflection: null,
            error: handleApiError(error).message
        });
    }
});

// API proxy endpoints for AJAX calls
app.delete('/api/memory/clear', async (req, res) => {
    try {
        const response = await apiClient.delete('/memory/clear');
        res.json(response.data);
    } catch (error) {
        res.status(error.response?.status || 500).json(handleApiError(error));
    }
});

app.get('/api/patterns/:keyword', async (req, res) => {
    try {
        const response = await apiClient.get(`/patterns/${req.params.keyword}`);
        res.json(response.data);
    } catch (error) {
        res.status(error.response?.status || 500).json(handleApiError(error));
    }
});

// Start server
app.listen(PORT, () => {
    console.log('🌐 AI Core Frontend Server');
    console.log('==========================');
    console.log(`✅ Frontend: http://localhost:${PORT}`);
    console.log(`🔗 Backend:  ${BACKEND_URL}`);
    console.log(`🔑 Token:    ${BEARER_TOKEN ? 'Configured' : 'Missing!'}`);
    console.log('');
    console.log('📝 Open your browser at http://localhost:' + PORT);
});
