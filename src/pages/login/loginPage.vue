<template>
    <div class="login_window">
        <div class="login_box">
            <div class="login_title">Login Mark Tie</div>
        </div>
    </div>
</template>

<script setup>

</script>

<style lang="scss">
.login_window {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;

    .login_box {
        width: 50%;
        height: 40%;
        background: rgba(255, 255, 255, 0.5);
        box-shadow: 5px 30px 25px rgba(0, 0, 0, 0.25);
        border-radius: 10px;
        transition: all 300ms;
        
        .login_title{
        }

        &:hover {
            transition: all 300ms;
            width: 51%;
            height: 41%;
        }
    }
}

</style>