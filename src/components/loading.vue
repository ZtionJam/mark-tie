<template>
    <div class="loading-overlay" v-if="visible">
        <div class="loading-spinner">
            <div class="spinner"></div>
        </div>
        <div class="text">{{ text }}</div>
    </div>
</template>

<script>
export default {
    name: 'Loading',
    props: {
        visible: Boolean,
        text: String,
    },
}
</script>

<style scoped>
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
}

.loading-spinner {
    width: 60px;
    height: 60px;
}

.text {
    text-align: center;
    width: 100px;
    margin-top: 25px;
    font-weight: bold;
    color: white;
}

.spinner {
    border: 10px solid rgba(255, 255, 255, 0.3);
    border-top-color: #fff;
    border-radius: 50%;
    width: 100%;
    height: 100%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}
</style>