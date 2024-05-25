import ultralytics

ultralytics.YOLO("./weights/car_detect.pt").export(format="onnx", simplify=True, imgsz=(1024, 1280))
ultralytics.YOLO("./weights/car_classification.pt").export(format="onnx", simplify=True, imgsz=(1024, 1280))
