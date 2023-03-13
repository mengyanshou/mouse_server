import 'dart:io';

Future<void> main(List<String> args) async {
  Socket socket=await Socket.connect('127.0.0.1', 26541);
  socket.listen((event) { 
    print('event: $event');
  });
}