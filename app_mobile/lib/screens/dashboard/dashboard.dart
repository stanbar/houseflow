import 'package:app_mobile/services/device.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Dashboard extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<DeviceService>(
      builder: (context, deviceModel, child) {
        return Column(children: [
          Expanded(
            child: ListView.builder(
                itemCount: deviceModel.firebaseDevices.length,
                itemBuilder: (context, index) {
                  final device = deviceModel.firebaseDevices[index];
                  final exists = deviceModel.activeDevices
                      .any((element) => element.uid == device.uid);
                  return Text("${device.type} Active: $exists");
                }),
          ),
        ]);
      },
    );
  }
}