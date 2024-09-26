<template>
  <div class="container">
    <div class="topic_area">
      <div class="title"><img class="bar_icon" src="../../assets/img/avatar.jpg" alt="">
        <p>孙笑川吧</p></div>
      <div class="topic_box">
        <div class="content_box">
          <div class="t_title"><p>一行代码是保图片不会随着单元格的大小变化而改变大格的大小变化而改变大</p></div>
          <div class="t_content"><p>
            这一行代码是为了确保图片不会随着单元格的大小变化而改变大小，即图片的尺寸固定。如果希望图片能够随单元格大小自动缩放，可以不设置这一行或设置为其他类型的锚点。</p>
          </div>
          <div class="t_img">
            <div><img src="../../assets/img/man_gun.jpeg" alt=""></div>
            <div><img src="../../assets/img/man_gun.jpeg" alt=""></div>
          </div>
          <div class="t_send">
            <p>IP属地：上海 2024-12-09 12:00:00</p>
          </div>
        </div>
        <div class="author_box">
          <div class="user_box"></div>
          <div class="forward_box"></div>
        </div>
      </div>
    </div>
    <div class="comment_area">

    </div>
  </div>
</template>
<script setup>

</script>
<style scoped lang="scss">
.container {
  width: 100%;
  height: 100%;
  background: url("../../assets/img/tie_bg.png") no-repeat;
  background-size: cover;

  padding: 15px;

  .topic_area {
    width: 100%;
    margin-top: 10px;

    .title {
      width: 100%;

      .bar_icon {
        width: 20px;
        float: left;
        border-radius: 20px;
        margin-top: 2px;
      }

      p {
        font-size: 16px;
        line-height: 30px;
        text-indent: 10px;
      }
    }

    .topic_box {
      display: flex;
      flex-direction: row;
      justify-content: space-between;

      .content_box {
        width: 70%;
        background: white;
        border-radius: 10px;

        .t_title {
          p {
            padding: 10px;
            text-indent: 20px;
            font-size: 15px;
            font-weight: bold;
          }
        }

        .t_content {
          p {
            padding-left: 10px;
            padding-right: 10px;
            text-indent: 20px;
            font-size: 14px;
          }
        }

        .t_img {
          display: flex;
          justify-content: flex-start;
          flex-wrap: wrap;
          width: 100%;
          padding: 10px;

          > div {
            overflow: hidden;
            height: 125px;
            border-radius: 5px;
            border: 1px solid #ccc;
            margin: 0 10px 10px 0;
            box-sizing: border-box;
            background-color: #999;
            flex: 0 0 calc((100% - 40px) / 3);


            img {
              width: 100%;
            }
          }
        }

        .t_send {
          width: 100%;

          p {
            font-size: 12px;
            color: #ccc;
            line-height: 25px;
            text-align: right;
            margin-right: 15px;
          }
        }

      }

      .author_box {
        height: 230px;
        width: 28%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;

        .user_box {
          background: white;
          height: 180px;
          border-radius: 10px;
        }

        .forward_box {
          background: white;
          height: 40px;
          border-radius: 10px;
        }
      }
    }

  }
}
</style>