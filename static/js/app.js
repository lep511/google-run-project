// Chat application logic
class ChatApp {
    constructor() {
        this.chatMessages = document.getElementById('chatMessages');
        this.messageInput = document.getElementById('messageInput');
        this.sendBtn = document.getElementById('sendBtn');
        this.loading = document.getElementById('loading');
        
        this.initializeEventListeners();
    }
    
    initializeEventListeners() {
        // Send button click
        this.sendBtn.addEventListener('click', () => this.sendMessage());
        
        // Enter key press
        this.messageInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                this.sendMessage();
            }
        });
        
        // Auto-focus input on load
        this.messageInput.focus();
    }
    
    async sendMessage() {
        const message = this.messageInput.value.trim();
        
        if (!message) return;
        
        // Add user message to chat
        this.addMessage(message, 'user');
        
        // Clear input and show loading
        this.messageInput.value = '';
        this.setLoading(true);
        
        try {
            const response = await this.callChatAPI(message);
            this.addMessage(response.response, 'ai');
        } catch (error) {
            console.error('Chat API error:', error);
            this.addMessage(
                'Sorry, I encountered an error. Please try again.',
                'ai'
            );
        } finally {
            this.setLoading(false);
        }
    }
    
    async callChatAPI(message) {
        const response = await fetch('/api/chat', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ message: message })
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        return await response.json();
    }
    
    addMessage(content, sender) {
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${sender}-message`;
        
        const senderLabel = sender === 'user' ? 'You' : 'AI Assistant';
        messageDiv.innerHTML = `<strong>${senderLabel}:</strong> ${this.escapeHtml(content)}`;
        
        this.chatMessages.appendChild(messageDiv);
        this.scrollToBottom();
    }
    
    setLoading(isLoading) {
        this.sendBtn.disabled = isLoading;
        this.loading.style.display = isLoading ? 'block' : 'none';
        
        if (!isLoading) {
            this.messageInput.focus();
        }
    }
    
    scrollToBottom() {
        this.chatMessages.scrollTop = this.chatMessages.scrollHeight;
    }
    
    escapeHtml(unsafe) {
        return unsafe
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&#039;");
    }
}

// Utility functions for backward compatibility
function sendMessage() {
    chatApp.sendMessage();
}

function handleKeyPress(event) {
    if (event.key === 'Enter') {
        chatApp.sendMessage();
    }
}

// Initialize the chat application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.chatApp = new ChatApp();
    console.log('Chat application initialized');
});

// Add some visual feedback for better UX
document.addEventListener('DOMContentLoaded', () => {
    // Add typing animation
    const style = document.createElement('style');
    style.textContent = `
        .typing-indicator {
            display: inline-block;
        }
        
        .typing-indicator span {
            display: inline-block;
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background-color: #999;
            margin: 0 2px;
            animation: typing 1.4s infinite ease-in-out;
        }
        
        .typing-indicator span:nth-child(1) { animation-delay: -0.32s; }
        .typing-indicator span:nth-child(2) { animation-delay: -0.16s; }
        
        @keyframes typing {
            0%, 80%, 100% {
                transform: scale(0.8);
                opacity: 0.5;
            }
            40% {
                transform: scale(1);
                opacity: 1;
            }
        }
    `;
    document.head.appendChild(style);
});