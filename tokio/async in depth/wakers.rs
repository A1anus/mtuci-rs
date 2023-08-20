fn poll(self: Pin<&mut Self>, cx: &mut Context)
    -> Poll<Self::Output>;